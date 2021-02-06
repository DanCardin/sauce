function _{0}_autoload {{
  {0} --autoload
}}

function _{0}_autoload_precmd {{
  _{0}_autoload

  precmd_functions=(${{(@)precmd_functions:#_{0}_autoload_precmd}})
  builtin unfunction _{0}_autoload_precmd
}}

add-zsh-hook chpwd _{0}_autoload
add-zsh-hook precmd _{0}_autoload_precmd
