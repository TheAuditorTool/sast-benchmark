require_relative 'shared'

def redirect_rack_location(req)
  [302, { 'Location' => req.param('url') }, ['']]
end
