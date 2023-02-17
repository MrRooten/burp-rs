require 'json'
class RBModule_b
    def initialize
    end
    def metadata
        info = { "name" => "b", 
            "description" => "test b" }
        return info
    end

    def passive_run(index)
        #scan("http://127.0.0.1:8009")
        #puts "b is running"
        #sleep(2)
        #puts "b is running2"
        resp = Request.get("http://127.0.0.1:8009/index.php")
        #puts resp.inspect
    end
    def active_run(url, opts)
    end
end