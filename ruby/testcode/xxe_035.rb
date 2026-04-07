require_relative 'shared'
require 'rexml/document'
require 'rexml/streamlistener'
require 'rexml/parsers/streamparser'

# vuln-code-snippet start ruby_xxe_multi_doc_stream
def parse_multi_doc_stream_handler(req)
  xml = req.post('xml')
  class MyStreamListener; include REXML::StreamListener; end
  REXML::Document.parse_stream(xml, MyStreamListener.new) # vuln-code-snippet vuln-line ruby_xxe_multi_doc_stream
  BenchmarkResponse.ok('parsed')
end
# vuln-code-snippet end ruby_xxe_multi_doc_stream
