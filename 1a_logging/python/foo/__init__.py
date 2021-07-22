import logging

from . import bar

log = logging.getLogger(__name__)


def run():
    log.warn("warn")
    log.info("info")
    log.debug("debug")
    bar.run()
