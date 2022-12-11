class RBModule
    def initialize
    end
    def metadata
        grades = { "name" => "thinkphp_lang_rce", 
            "description" => "" }
        return grades
    end

    def passive_run(index)
        client = RBHttpClient.new
        request = {
            "method" => "get",
            "url" => "https://cn.baidu.com"
        }
        print("hello123: ")
            #... process, may raise an exception
        
        begin
            puts client.send(request)
        rescue Exception => e
            puts "#{e}"
            raise e
        end
        
        puts "after client"
        
    end

end