function {0}_autoload --on-variable PWD;
  {0} --autoload --autoload-previous "$dirprev[-1]" | source
end
