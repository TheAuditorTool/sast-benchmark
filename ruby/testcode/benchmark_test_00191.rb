require_relative 'shared'
require 'nokogiri'

def parse_html5_handler(req)
  input = req.post('data')
  doc = Nokogiri::HTML5.parse(input)
  BenchmarkResponse.json({ title: doc.at('title')&.text })
end
