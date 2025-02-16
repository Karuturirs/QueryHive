GET _cat/indices

PUT /query-hive-index


#### Cerate Chunker index with mapping

```cmd
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
```

#### Cerate pipeline chunker to break the content


```cmd
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
```