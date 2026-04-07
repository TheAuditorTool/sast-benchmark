require_relative 'shared'

def handler(req)
  service_name = case req.param('type')
                 when 'pdf' then 'PdfService'
                 when 'csv' then 'CsvService'
                 else raise ArgumentError, 'unknown type'
                 end
  BenchmarkResponse.json({ service: service_name })
end
