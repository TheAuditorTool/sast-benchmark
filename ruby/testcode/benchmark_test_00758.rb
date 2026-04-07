require_relative 'shared'

def select_handler(req)
  type = req.param('type')
  handler = case type
            when 'json' then :handle_json
            when 'xml' then :handle_xml
            when 'csv' then :handle_csv
            else return BenchmarkResponse.bad_request('unknown')
            end
  BenchmarkResponse.ok("handler: #{handler}")
end
