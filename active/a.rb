require 'json'
class RBModule_a
    def initialize
    end
    def metadata
        info = { "name" => "a", 
            "description" => "test a" }
        return info
    end

    def passive_run(index)
        #scan("http://127.0.0.1:8009")
        #puts "a is running"
        #sleep(2)
        #puts "a is running2"
        resp = Request.get("http://127.0.0.1:8009")
        #puts resp.inspect
    end

    def active_run(url, opts)
    end

end
