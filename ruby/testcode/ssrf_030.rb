require_relative 'shared'

module ElasticsearchClient
  def self.new(host:); self; end
  def self.ping; true; end
end

# vuln-code-snippet start ruby_ssrf_elasticsearch_host
def ping_elasticsearch(req)
  ElasticsearchClient.new(host: req.param('es_host')).ping # vuln-code-snippet vuln-line ruby_ssrf_elasticsearch_host
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_elasticsearch_host
