require_relative 'shared'

def load_dynamic_erb(req)
  name = req.param('name')
  load("templates/#{name}.rb")
  BenchmarkResponse.ok('template loaded')
end
