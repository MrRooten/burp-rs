class HistoryLog
    @@history = RBHttpLog.new
    def self.get_req(index)
        return LogRequest.new(@@history.get_http_req(index))
    end

   
    def self.get_resp(index)
        return LogResponse.new(@@history.get_http_resp(index))
    end
end

class LogRequest
    def initialize(hash)
        @method = hash['method']
        @url = hash['url']
        @headers = hash['headers']
        @body = hash['body']
    end

    def url 
        @url
    end

    def headers 
        @headers
    end

    def body
        @body
    end

    def method
        @method
    end
end

class LogResponse
    def initialize(hash)
        @status = hash['status_code']
        @url = hash['url']
        @headers = hash['headers']
        @body = hash['body']
        @request = LogRequest.new(hash['request'])
    end

    def headers 
        @headers
    end

    def body
        @body
    end

    def url 
        @url
    end

    def request
        @request
    end

    def status 
        @status
    end
end