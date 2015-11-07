set cmd to "function make_meta(){a={\"release_date\":document.getElementsByClassName('released')[0].children[1].textContent,\"catalog\":document.getElementsByClassName('catalogno')[0].children[1].textContent};a['comment']=`\\n ${a.catalog},${document.baseURI},\\n`;return a};JSON.stringify(make_meta());"

tell application "Safari"
	set res to do JavaScript cmd in current tab of first window
end tell

-- set the clipboard to res

