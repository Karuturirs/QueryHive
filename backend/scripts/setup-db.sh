curl -X PUT "http://localhost:9200/documents" -H "Content-Type: application/json" -d '
{
  "mappings": {
    "properties": {
      "title": { "type": "text" },
      "content": { "type": "text" },
      "path": { "type": "text" },
      "tags": { "type": "keyword" },
      "created_at": { "type": "date" },
    }
  }
}'


