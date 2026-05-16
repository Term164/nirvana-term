#!/bin/bash

source .env

curl --request POST \
    --url "${JIRA_BASE_URL}/rest/api/3/issue/DEV-1?fields=summary" \
    --user "${JIRA_USER_EMAIL}:${JIRA_CLOUD_API}" \
    --header 'Accept: application/json' \
    --header 'Content-Type: application/json' \
    --data '{
        "started": "2026-05-01T09:00:00.000+0000",
        "timeSpent": "1h 30m",
        "comment": {
            "content": [
                {
                    "content": [
                        {
                            "text": "I did some work here.",
                            "type": "text"
                        }
                    ],
                    "type": "paragraph"
                }
            ],
            "type": "doc",
            "version": 1
        }
    }' | jq
