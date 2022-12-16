class Request
    def self.get(url, headers={}, body=[])
        request = {}
        request["method"] = "get"
        request["url"] = url
        headers_req = {}
        headers.each do |key, value|
            headers_req[key] = value
        end
        request["headers"] = headers_req
        request["body"] = body 
        client = RBHttpClient.new
        response = Response.new(client.send(request))
        return response
    end

    def self.post(url, headers={}, body="")
        request = {}
        request["url"] = url
        request["headers"] = headers
        request["body"] = body
        client = RBHttpClient.new
        response = Response.new(client.send(request))
    end
end

class Response 
    def initialize(resp)
        @orig_resp = resp
        @status = resp["status_code"]
        @headers = resp["headers"]
        @body = resp["body"]
        @request = resp["request"]
    end

    def body
        @body
    end

    def request
        @request
    end

    def headers
        @headers
    end

    def status 
        @status
    end

    def orig_resp 
        @orig_resp
    end
end