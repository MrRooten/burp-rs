class RBModule
    def initialize
    end
    def metadata
        grades = { "name" => "test1", "description" => "hello, I'm test1" }
        return grades
    end

    def passive_run(index)
        a = RBHttpLog.new
        puts a.get_http_req(index)
        puts a.get_http_resp(index)
        puts self.metadata
        puts "Scan payload test1"
    end
end

