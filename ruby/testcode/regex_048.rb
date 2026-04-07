require_relative 'shared'
require 'strscan'

# vuln-code-snippet start ruby_regex_stringscanner_fixed
def handle_stringscanner_fixed(req)
  input = req.param('input')
  scanner = StringScanner.new(input)
  digits = scanner.scan(/\d+/) # vuln-code-snippet safe-line ruby_regex_stringscanner_fixed
  BenchmarkResponse.json({ digits: digits })
end
# vuln-code-snippet end ruby_regex_stringscanner_fixed
