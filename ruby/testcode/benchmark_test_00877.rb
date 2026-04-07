require_relative 'shared'

def parse_stripped(req)
  xml_input = req.body_str
  clean_xml = xml_input.gsub(/<!DOCTYPE[^>]*>/i, '')
  BenchmarkResponse.ok(clean_xml)
end
