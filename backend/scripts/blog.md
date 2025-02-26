Vector search is a powerful way to search data based on meaning rather than exact or inexact token matching techniques. However the text embedding models that power vector search can only process short passages of text on the order of several sentences rather than BM25 based techniques that can work on arbitrarily large amounts of text. Combining large documents seamlessly with vector search is now possible with Elasticsearch.
How does it work at a high level?

The combination of Elasticsearch features such as ingest pipelines, the flexibility of a script processor and new support for nested documents with dense_vectors allows for a straightforward way to at ingest time chunk large documents into small enough passages that can then be processed by text embedding models to generate all the vectors needed to represent the full meaning of the large documents.

Ingest your document data as you would normally, and add to your ingest pipeline a script processor to break the large text data into an array of sentence or other types of chunks followed by a for_each processor to run an inference processor on each chunk. Mappings for the index are defined such that the array of chunks is set up as a nested object with a dense_vector mapping as a subobject which will then properly index each of the vectors and make them searchable.
How to chunk large documents via ingest pipelines & nested vectors
Load a text embedding model

The first thing you will need is a model to create the text embeddings out of the chunks, you can use whatever you would like, but this example will run end to end on the all-distilroberta-v1 model. With an Elastic Cloud cluster created or another Elasticsearch cluster ready, we can upload the text embedding model using the eland library.

MODEL_ID = "sentence-transformers/all-MiniLM-L6-v2"
ELASTIC_PASSWORD = "YOURPASSWORD"
CLOUD_ID = "YOURCLOUDID"

eland_import_hub_model \
    --cloud-id $CLOUD_ID \
    --es-username elastic \
    --es-password $ELASTIC_PASSWORD \
    --hub-model-id $MODEL_ID \
    --task-type text_embedding \
    --start

Mappings example

Next step is to prepare the mappings to handle the array of sentences and vector objects that will be created during the ingest pipeline. For this particular text embedding model the dimensions are 384 and dot_product similarity will be used for nearest neighbor calculations:

PUT chunker
{
  "mappings": {
    "dynamic": "true",
    "properties": {
      "passages": {
        "type": "nested",
        "properties": {
          "vector": {
            "properties": {
              "predicted_value": {
                "type": "dense_vector",
                "index": true,
                "dims": 384,
                "similarity": "dot_product"
              }
            }
          }
        }
      }
    }
  }
}

Ingest pipeline examples

The last preparation step is to define an ingest pipeline to break up the body_content field into chunks of text stored in the passages field. This pipeline has two processors, the first script processor breaks up the body_content field into an array of sentences stored in the passages field via a regular expression. For further research read up on regular expression advanced features such as negative lookbehind and positive lookbehind to understand how it tries to properly split on sentence boundaries, not split on Mr. or Mrs. or Ms., and keep the punctuation with the sentence. It also tries to concatenate the sentence chunks back together as long as the total string length is under the parameter passed to the script. The next for each processor runs the text embedding model on each sentence via an inferrence processor:

PUT _ingest/pipeline/chunker
{
  "processors": [
    {
      "script": {
        "description": "Chunk body_content into sentences by looking for . followed by a space",
        "lang": "painless",
        "source": """
          String[] envSplit = /((?<!M(r|s|rs)\.)(?<=\.) |(?<=\!) |(?<=\?) )/.split(ctx['body_content']);
          ctx['passages'] = new ArrayList();
          int i = 0;
          boolean remaining = true;
          if (envSplit.length == 0) {
            return
          } else if (envSplit.length == 1) {
            Map passage = ['text': envSplit[0]];ctx['passages'].add(passage)
          } else {
            while (remaining) {
              Map passage = ['text': envSplit[i++]];
              while (i < envSplit.length && passage.text.length() + envSplit[i].length() < params.model_limit) {passage.text = passage.text + ' ' + envSplit[i++]}
              if (i == envSplit.length) {remaining = false}
              ctx['passages'].add(passage)
            }
          }
          """,
        "params": {
          "model_limit": 400
        }
      }
    },
    {
      "foreach": {
        "field": "passages",
        "processor": {
          "inference": {
            "field_map": {
              "_ingest._value.text": "text_field"
            },
            "model_id": "sentence-transformers__all-minilm-l6-v2",
            "target_field": "_ingest._value.vector",
            "on_failure": [
              {
                "append": {
                  "field": "_source._ingest.inference_errors",
                  "value": [
                    {
                      "message": "Processor 'inference' in pipeline 'ml-inference-title-vector' failed with message '{{ _ingest.on_failure_message }}'",
                      "pipeline": "ml-inference-title-vector",
                      "timestamp": "{{{ _ingest.timestamp }}}"
                    }
                  ]
                }
              }
            ]
          }
        }
      }
    }
  ]
}

Add some documents

Now we can add documents with large amounts of text in body_content and automatically have them chunked, and each chunk text embedded into vectors by the model:

PUT chunker/_doc/1?pipeline=chunker
{
"title": "Adding passage vector search to Lucene",
"body_content": "Vector search is a powerful tool in the information retrieval tool box. Using vectors alongside lexical search like BM25 is quickly becoming commonplace. But there are still a few pain points within vector search that need to be addressed. A major one is text embedding models and handling larger text input. Where lexical search like BM25 is already designed for long documents, text embedding models are not. All embedding models have limitations on the number of tokens they can embed. So, for longer text input it must be chunked into passages shorter than the model’s limit. Now instead of having one document with all its metadata, you have multiple passages and embeddings. And if you want to preserve your metadata, it must be added to every new document. A way to address this is with Lucene's “join” functionality. This is an integral part of Elasticsearch’s nested field type. It makes it possible to have a top-level document with multiple nested documents, allowing you to search over nested documents and join back against their parent documents. This sounds perfect for multiple passages and vectors belonging to a single top-level document! This is all awesome! But, wait, Elasticsearch doesn’t support vectors in nested fields. Why not, and what needs to change? The key issue is how Lucene can join back to the parent documents when searching child vector passages. Like with kNN pre-filtering versus post-filtering, when the joining occurs determines the result quality and quantity. If a user searches for the top four nearest parent documents (not passages) to a query vector, they usually expect four documents. But what if they are searching over child vector passages and all four of the nearest vectors are from the same parent document? This would end up returning just one parent document, which would be surprising. This same kind of issue occurs with post-filtering."
}

PUT chunker/_doc/3?pipeline=chunker
{
"title": "Automatic Byte Quantization in Lucene",
"body_content": "While HNSW is a powerful and flexible way to store and search vectors, it does require a significant amount of memory to run quickly. For example, querying 1MM float32 vectors of 768 dimensions requires roughly 1,000,000∗4∗(768+12)=3120000000≈31,000,000∗4∗(768+12)=3120000000bytes≈3GB of ram. Once you start searching a significant number of vectors, this gets expensive. One way to use around 75% less memory is through byte quantization. Lucene and consequently Elasticsearch has supported indexing byte vectors for some time, but building these vectors has been the user's responsibility. This is about to change, as we have introduced int8 scalar quantization in Lucene. All quantization techniques are considered lossy transformations of the raw data. Meaning some information is lost for the sake of space. For an in depth explanation of scalar quantization, see: Scalar Quantization 101. At a high level, scalar quantization is a lossy compression technique. Some simple math gives significant space savings with very little impact on recall. Those used to working with Elasticsearch may be familiar with these concepts already, but here is a quick overview of the distribution of documents for search. Each Elasticsearch index is composed of multiple shards. While each shard can only be assigned to a single node, multiple shards per index gives you compute parallelism across nodes. Each shard is composed as a single Lucene Index. A Lucene index consists of multiple read-only segments. During indexing, documents are buffered and periodically flushed into a read-only segment. When certain conditions are met, these segments can be merged in the background into a larger segment. All of this is configurable and has its own set of complexities. But, when we talk about segments and merging, we are talking about read-only Lucene segments and the automatic periodic merging of these segments. Here is a deeper dive into segment merging and design decisions."
}

PUT chunker/_doc/2?pipeline=chunker
{
"title": "Use a Japanese language NLP model in Elasticsearch to enable semantic searches",
"body_content": "Quickly finding necessary documents from among the large volume of internal documents and product information generated every day is an extremely important task in both work and daily life. However, if there is a high volume of documents to search through, it can be a time-consuming process even for computers to re-read all of the documents in real time and find the target file. That is what led to the appearance of Elasticsearch and other search engine software. When a search engine is used, search index data is first created so that key search terms included in documents can be used to quickly find those documents. However, even if the user has a general idea of what type of information they are searching for, they may not be able to recall a suitable keyword or they may search for another expression that has the same meaning. Elasticsearch enables synonyms and similar terms to be defined to handle such situations, but in some cases it can be difficult to simply use a correspondence table to convert a search query into a more suitable one. To address this need, Elasticsearch 8.0 released the vector search feature, which searches by the semantic content of a phrase. Alongside that, we also have a blog series on how to use Elasticsearch to perform vector searches and other NLP tasks. However, up through the 8.8 release, it was not able to correctly analyze text in languages other than English. With the 8.9 release, Elastic added functionality for properly analyzing Japanese in text analysis processing. This functionality enables Elasticsearch to perform semantic searches like vector search on Japanese text, as well as natural language processing tasks such as sentiment analysis in Japanese. In this article, we will provide specific step-by-step instructions on how to use these features."
}

PUT chunker/_doc/5?pipeline=chunker
{
"title": "We can chunk whatever we want now basically to the limits of a document ingest",
"body_content": """Chonk is an internet slang term used to describe overweight cats that grew popular in the late summer of 2018 after a photoshopped chart of cat body-fat indexes renamed the "Chonk" scale grew popular on Twitter and Reddit. Additionally, "Oh Lawd He Comin'," the final level of the Chonk Chart, was adopted as an online catchphrase used to describe large objects, animals or people. It is not to be confused with the Saturday Night Live sketch of the same name. The term "Chonk" was popularized in a photoshopped edit of a chart illustrating cat body-fat indexes and the risk of health problems for each class (original chart shown below). The first known post of the "Chonk" photoshop, which classifies each cat to a certain level of "chonk"-ness ranging from "A fine boi" to "OH LAWD HE COMIN," was posted to Facebook group THIS CAT IS C H O N K Y on August 2nd, 2018 by Emilie Chang (shown below). The chart surged in popularity after it was tweeted by @dreamlandtea[1] on August 10th, 2018, gaining over 37,000 retweets and 94,000 likes (shown below). After the chart was posted there, it began growing popular on Reddit. It was reposted to /r/Delighfullychubby[2] on August 13th, 2018, and /r/fatcats on August 16th.[3] Additionally, cats were shared with variations on the phrase "Chonk." In @dreamlandtea's Twitter thread, she rated several cats on the Chonk scale (example, shown below, left). On /r/tumblr, a screenshot of a post featuring a "good luck cat" titled "Lucky Chonk" gained over 27,000 points (shown below, right). The popularity of the phrase led to the creation of a subreddit, /r/chonkers,[4] that gained nearly 400 subscribers in less than a month. Some photoshops of the chonk chart also spread on Reddit. For example, an edit showing various versions of Pikachu on the chart posted to /r/me_irl gained over 1,200 points (shown below, left). The chart gained further popularity when it was posted to /r/pics[5] September 29th, 2018."""
}

Search those documents

To search the data and return what chunk matched the query best you use inner_hits with the knn clause to return just that best matching chunk of the document in the hits output from the query:

GET chunker/_search
{
  "_source": false,
  "fields": [
    "title"
  ],
  "knn": {
    "inner_hits": {
      "_source": false,
      "fields": [
        "passages.text"
      ]
    },
    "field": "passages.vector.predicted_value",
    "k": 1,
    "num_candidates": 100,
    "query_vector_builder": {
      "text_embedding": {
        "model_id": "sentence-transformers__all-minilm-l6-v2",
        "model_text": "Can I use multiple vectors per document now?"
      }
    }
  }
}

Will return the best document and the relevant portion of the larger document text:

{
  "took": 4,
  "timed_out": false,
  "_shards": {
    "total": 1,
    "successful": 1,
    "skipped": 0,
    "failed": 0
  },
  "hits": {
    "total": {
      "value": 1,
      "relation": "eq"
    },
    "max_score": 0.75261426,
    "hits": [
      {
        "_index": "chunker",
        "_id": "1",
        "_score": 0.75261426,
        "_ignored": [
          "body_content.keyword",
          "passages.text.keyword"
        ],
        "fields": {
          "title": [
            "Adding passage vector search to Lucene"
          ]
        },
        "inner_hits": {
          "passages": {
            "hits": {
              "total": {
                "value": 1,
                "relation": "eq"
              },
              "max_score": 0.75261426,
              "hits": [
                {
                  "_index": "chunker",
                  "_id": "1",
                  "_nested": {
                    "field": "passages",
                    "offset": 3
                  },
                  "_score": 0.75261426,
                  "fields": {
                    "passages": [
                      {
                        "text": [
                          "This sounds perfect for multiple passages and vectors belonging to a single top-level document! This is all awesome! But, wait, Elasticsearch doesn’t support vectors in nested fields. Why not, and what needs to change? The key issue is how Lucene can join back to the parent documents when searching child vector passages."
                        ]
                      }
                    ]
                  }
                }
              ]
            }
          }
        }
      }
    ]
  }
}

Review

The approach used here shows the power of leveraging the different capabilities of Elasticsearch to solve a larger problem.

Ingest pipelines allow you to preprocess your documents before indexing, and while there are many processors that do specific targeted tasks, sometimes you need the power of a scripting language to be able to do things like break up text into an array of sentences. Because you can access the document before it is indexed you have the ability to remake the data in nearly any fashion you can imagine as long as all the information is within the document itself. The foreach processor allows us to wrap something that may run zero to N times without knowing in advance how many times it needs to execute. In this case we are using it to run over as many sentences as we extract to run the infer processor to make vectors.

The mappings of the index are prepared to handle the array of now objects of text and vector that did not exist in the original document with a nested object which indexes the data in a way that we can properly search the document.

Using knn with nested support for vectors allows the use of inner_hits to present the best scoring portion of the document which can substitute for what would be usually done via highlighting in a BM25 query.