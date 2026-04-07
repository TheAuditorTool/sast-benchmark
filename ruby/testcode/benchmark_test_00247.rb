require_relative 'shared'

module ElasticsearchClient
  def self.new(host:); self; end
  def self.ping; true; end
end

def ping_elasticsearch(req)
  ElasticsearchClient.new(host: req.param('es_host')).ping
  BenchmarkResponse.json({ ok: true })
end
