class HttpClient
    def get(url, headers={}, body=[])
        request = {}
        request["method"] = "get"
        request["url"] = url
        request["headers"] = headers
        request["body"] = body 
        client = RBHttpClient.new
        response = client.send(request)
        return response
    end

    def post(url, headers={}, body="")
        request = {}
        request["url"] = url
        request["headers"] = headers
        request["body"] = body
    end
end