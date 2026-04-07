require_relative 'shared'

def eval_template(req)
  template_name = req.param('template')
  binding.eval(File.read("templates/#{template_name}.rb"))
  BenchmarkResponse.ok('rendered')
end
