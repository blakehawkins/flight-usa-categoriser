#!/usr/bin/env bash

xsv select From,To data.csv | \
	tail -n +2 | \
	tr -s ',' ' ' | \
	xargs -I {} bash -c "echo {} | cut -d' ' -f1 && echo {} | cut -d' ' -f2" | \
	sort | \
	uniq
