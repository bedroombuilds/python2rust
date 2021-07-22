import logging
import logging.config
import yaml

import foo

if __name__ == "__main__":
    with open('logconf.yaml', 'r') as f:
        log_cfg = yaml.safe_load(f.read())
    logging.config.dictConfig(log_cfg)
    logging.warning('warn')
    logging.error("error")
    logging.info('info')
    logging.debug("debug")
    foo.run()
