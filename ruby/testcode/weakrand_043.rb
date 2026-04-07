require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_rand_non_security
def get_featured_items(req)
  items = %w[item_a item_b item_c item_d item_e item_f item_g item_h item_i item_j]
  position = rand(10) # vuln-code-snippet safe-line ruby_weakrand_rand_non_security
  BenchmarkResponse.json({ items: items.rotate(position).first(5) })
end
# vuln-code-snippet end ruby_weakrand_rand_non_security
