require_relative 'shared'

class UserInput < T::Struct
  const :name, String
  const :age,  Integer
end

# vuln-code-snippet start ruby_deser_sorbet_struct
def sorbet_struct_handler(req)
  data = JSON.parse(req.post('data'))  # vuln-code-snippet safe-line ruby_deser_sorbet_struct
  user = UserInput.new(**data.transform_keys(&:to_sym))
  BenchmarkResponse.json({ result: user.name })
end
# vuln-code-snippet end ruby_deser_sorbet_struct
