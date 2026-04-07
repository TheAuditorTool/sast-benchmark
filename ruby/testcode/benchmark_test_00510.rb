require_relative 'shared'

def setup_object(req)
  setup_code = req.param('setup')
  obj = Object.new.tap { |o| o.instance_eval(setup_code) }
  BenchmarkResponse.json({ object_id: obj.object_id })
end
