cd ..
git ls-files | awk '!/target/'  | xargs -n100 wc -l | awk -F ' +|\\.' \
    '/\./ { sumlines[$NF] += $2 } END { for (ext in sumlines) print ext, sumlines[ext] }' > stats/linecount.txt