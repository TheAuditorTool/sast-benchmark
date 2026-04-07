require_relative 'shared'

# vuln-code-snippet start ruby_reflect_case_factory
def handler(req)
  service_name = case req.param('type') # vuln-code-snippet safe-line ruby_reflect_case_factory
                 when 'pdf' then 'PdfService'
                 when 'csv' then 'CsvService'
                 else raise ArgumentError, 'unknown type'
                 end
  BenchmarkResponse.json({ service: service_name })
end
# vuln-code-snippet end ruby_reflect_case_factory
