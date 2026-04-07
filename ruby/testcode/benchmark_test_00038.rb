require_relative 'shared'
require 'nokogiri'

def parse_dtd_stripped_handler(req)
  xml = req.post('xml')
  clean_xml = xml.gsub(/<!DOCTYPE[^>]*>/, '')
  doc = Nokogiri::XML.parse(clean_xml) { |c| c.nonet }
  BenchmarkResponse.json({ root: doc.root&.name })
end
