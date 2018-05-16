run:
	kotlinc main.kt -include-runtime -d main.jar
	cp -f main.jar /tmp/main.jar
	cd /tmp && sudo dtrace -n 'pid$target::malloc:entry' -c '/usr/bin/java -jar main.jar'