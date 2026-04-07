require_relative 'shared'

def read_record; "reading"; end
def list_records; "listing"; end

def case_dispatch(req)
  result = case req.param('action')
           when 'read' then read_record
           when 'list' then list_records
           else raise ArgumentError, 'unknown action'
           end
  BenchmarkResponse.json({ result: result })
end
