require 'json'
class RBModule_test_issue
    def initialize
    end
    def metadata
        info = { "name" => "test_issue", 
            "description" => "" }
        return info
    end

    def passive_run(index)
        #scan("http://127.0.0.1:8009")
        resp = Request.get("http://127.0.0.1:8009")
        puts resp.inspect
    end

end
