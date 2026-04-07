require_relative 'shared'
require 'nokogiri'

def parse_html_xml_handler(req)
  xml = req.post('data')
  doc = Nokogiri::HTML(xml)
  BenchmarkResponse.json({ title: doc.at('title')&.text })
end
