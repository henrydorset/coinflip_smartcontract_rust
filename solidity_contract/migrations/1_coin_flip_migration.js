// 2_StoreValue_migration.js

const Migrations = artifacts.require("coinflip");

module.exports = function (deployer) {
  deployer.deploy(Migrations);
};