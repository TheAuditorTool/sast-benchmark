require_relative 'shared'
require 'nokogiri'

def parse_xslt_stylesheet_handler(req)
  xslt = Nokogiri::XSLT.parse(req.post('xslt'))
  doc = Nokogiri::XML('<root/>')
  result = xslt.transform(doc)
  BenchmarkResponse.html(result.to_s)
end
