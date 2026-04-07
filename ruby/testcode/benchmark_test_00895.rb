require_relative 'shared'

class InvoiceRecord
  def self.find_by_id(id)
    { id: id, amount: 9_999, customer: "acme_corp" }
  end
end

def current_graphql_user(ctx)
  ctx[:current_user]
end

def resolve_invoice(obj, args, ctx)
  invoice = InvoiceRecord.find_by_id(args[:id])
  invoice
end
