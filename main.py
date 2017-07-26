import re
import random
from telegram.ext import Updater, MessageHandler, BaseFilter, Filters

import config


class FilterPr(BaseFilter):
    def __init__(self):
        self.regex = re.compile(r'/(pr)+(@kotomei(_bot)?)?( |$)')

    def filter(self, message):
        return bool(message.text and self.regex.search(message.text) is not None)


class FilterPrOther(BaseFilter):
    def __init__(self):
        self.regex = re.compile(r'/(pr)+(@.+)')

    def filter(self, message):
        return bool(message.text and self.regex.search(message.text) is not None)


def pr(bot, update):
    if random.randint(1, 100) <= config.react_rate:
        update.message.reply_text(random.choice(config.react_emoticons))
    else:
        update.message.reply_text(config.default_emoticon)


def pr_other(bot, update):
    update.message.reply_text(config.other_emoticon)


def debug(bot, update):
    print(update.message.text)


def main():
    # logging.basicConfig(format='%(asctime)s - %(name)s - %(levelname)s - %(message)s', level=logging.INFO)

    updater = Updater(config.token)

    updater.dispatcher.add_handler(MessageHandler(FilterPr(), pr))
    updater.dispatcher.add_handler(MessageHandler(FilterPrOther(), pr_other))
    # updater.dispatcher.add_handler(MessageHandler(Filters.all, debug))

    updater.start_polling()
    updater.idle()


if __name__ == '__main__':
    main()
