require_relative 'shared'

class InvoiceRecord
  def self.find_by_id(id)
    { id: id, amount: 9_999, customer: "acme_corp" }
  end
end

def current_graphql_user(ctx)
  ctx[:current_user]
end

# vuln-code-snippet start ruby_authz_graphql_field
def resolve_invoice(obj, args, ctx)
  invoice = InvoiceRecord.find_by_id(args[:id]) # vuln-code-snippet vuln-line ruby_authz_graphql_field
  invoice
end
# vuln-code-snippet end ruby_authz_graphql_field
