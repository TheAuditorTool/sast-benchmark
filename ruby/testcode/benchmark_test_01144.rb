require_relative 'shared'

def get_featured_items(req)
  items = %w[item_a item_b item_c item_d item_e item_f item_g item_h item_i item_j]
  position = rand(10)
  BenchmarkResponse.json({ items: items.rotate(position).first(5) })
end
