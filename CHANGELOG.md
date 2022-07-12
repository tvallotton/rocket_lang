#  (2022-07-12)


### Bug Fixes

* clippy lints ([5ebbca9](https://github.com/tvallotton/rocket_lang/commit/5ebbca91c958f18ed5f82d59f8cbadf665f9ac31))
* Error Responder implementation ([ed2f86d](https://github.com/tvallotton/rocket_lang/commit/ed2f86dab9992c212771eec81e6037744921196c))


### Features

* add `custom_async` as an alternative to `custom` ([8714e6b](https://github.com/tvallotton/rocket_lang/commit/8714e6bf21b386245f2e6eefbf5bbb52a78a6978))
* change default value for unconfigured resolution. ([6abd16a](https://github.com/tvallotton/rocket_lang/commit/6abd16ab802328106a138e7dfbff25cac11fa512))


### BREAKING CHANGES
* Previously the Accept-Language header was used
  as a form of language resolution when the behavior
  isn't configured. Now the resolution defaults to
  English in these cases.



