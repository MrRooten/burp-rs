class RBModule
    def initialize
    end
    def metadata
        grades = { "name" => "thinkphp_lang_rce", 
            "description" => "" }
        return grades
    end

    def scan(url)
        client = HttpClient.new
        resp = client.get(url, headers={"host"=>"bing.com"})
        puts resp
        issue = {
            "name"=> "test_req",
            "level" => "info",
            "confidence" => "confirm",
            "detail" => "this is a detail",
            "host" => "https://baidu.com",
            "response" => resp
        }

        issuer = RBIssue.new
        issuer.push_issue(issue)
    end

    def passive_run(index)
        scan("https://baidu.com")
    end

end

