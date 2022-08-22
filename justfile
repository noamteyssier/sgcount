serve: clean
  bundle exec jekyll serve --incremental

clean:
  rm -rfv _site/
