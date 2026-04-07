require_relative 'shared'
require 'strscan'

def handle_stringscanner_fixed(req)
  input = req.param('input')
  scanner = StringScanner.new(input)
  digits = scanner.scan(/\d+/)
  BenchmarkResponse.json({ digits: digits })
end
