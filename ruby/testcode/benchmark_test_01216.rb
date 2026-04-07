require_relative 'shared'

class UserInput < T::Struct
  const :name, String
  const :age,  Integer
end

def sorbet_struct_handler(req)
  data = JSON.parse(req.post('data'))
  user = UserInput.new(**data.transform_keys(&:to_sym))
  BenchmarkResponse.json({ result: user.name })
end
