require_relative 'shared'

def extend_class(req)
  code_string = req.param('definition')
  String.class_eval(code_string)
  BenchmarkResponse.ok("class extended")
end
