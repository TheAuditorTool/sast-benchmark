require_relative 'shared'

FIXED_AVRO_SCHEMA_JSON = '{"type":"record","name":"User","fields":[{"name":"name","type":"string"}]}'

def avro_schema_deserialize_handler(req)
  schema = Avro::Schema.parse(FIXED_AVRO_SCHEMA_JSON)
  reader = Avro::IO::DatumReader.new(schema)
  obj = reader.read(Avro::IO::BinaryDecoder.new(StringIO.new(req.post('data'))))
  BenchmarkResponse.json({ result: obj })
end
