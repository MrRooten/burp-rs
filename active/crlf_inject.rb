class RBModule
    def initialize
    end
    def metadata
        grades = { "name" => "crlf_inject", 
            "description" => "" }
        return grades
    end

    def scan(uri)
        scheme = uri["scheme"]
        if scheme == nil 
            error("Scheme is none #{uri}")
            return 
        end
        #simeple test
        url = scheme + "://" + uri["host"] + uri["path"] + "%0D%0ASet-Cookie:testkey=testvalue"
        client = HttpClient.new
        resp = client.get(url, headers={"host"=>uri["host"]})
        headers = resp["headers"]
        if headers == nil 
            return 
        end
        #puts resp
        headers.each do |key,value|
            if key.casecmp("set-cookie")
                if value.contains("testkey")
                    info("Found cookie injection in #{url} #{"vuln".red}")
                    issue = {
                        "name"=> "crlf simple test",
                        "level" => "highrisk",
                        "confidence" => "confirm",
                        "detail" => "this is a detail",
                        "host" => uri["host"],
                        "response" => resp
                    }

                    IssueReporter.push_issue(issue)
                end
            end
        end


        if headers.each_value()
            issue = {
                "name"=> "test_req",
                "level" => "info",
                "confidence" => "confirm",
                "detail" => "this is a detail",
                "host" => url,
                "response" => resp
            }
        
            issuer = RBIssue.new
            issuer.push_issue(issue)
        end
        
    end

    def passive_run(index)
        log = HistoryLog.get_req index
        url = log['url']
        uri = UriParser.parse(url)
        info("Test url: #{uri}")
        scan(uri)
        #scan("http://127.0.0.1:8009")
    end

end