"""Elasticsearch cluster configuration -- IDENTICAL between vuln/ and safe/.

Represents an Elasticsearch cluster with no authentication (X-Pack security
disabled, default for ES < 8.0). The cluster is reachable only from within
the VPC, so engineers assumed it was safe. An SSRF vulnerability lets an
attacker use the application server as a proxy to query all indices.

Chain: attacker -> /search?es_url=http://10.0.1.5:9200/... -> index dump

Elasticsearch endpoints exposed:
  http://10.0.1.5:9200/_cat/indices?v
    -> Lists all indices with document counts
  http://10.0.1.5:9200/users/_search
    -> Full-text search across the users index (PII, hashed passwords)
  http://10.0.1.5:9200/_cluster/settings
    -> Cluster configuration including keystore entries
"""

ES_HOST = "http://10.0.1.5:9200"
ES_INDEX = "products"
ES_TIMEOUT = 5
