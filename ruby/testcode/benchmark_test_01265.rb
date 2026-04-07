require_relative 'shared'
require 'rexml/document'
require 'rexml/streamlistener'
require 'rexml/parsers/streamparser'

def parse_multi_doc_stream_handler(req)
  xml = req.post('xml')
  class MyStreamListener; include REXML::StreamListener; end
  REXML::Document.parse_stream(xml, MyStreamListener.new)
  BenchmarkResponse.ok('parsed')
end
