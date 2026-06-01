CONTENT="$(nine-poe --model "SearchForAIAgent" --prompt "$1")"
CONTENT=$(echo "$CONTENT" | sed 's/<br\/>/\n/g')

opencb send "$CONTENT"

